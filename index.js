const axios = require("axios");
const path = require("path");
const fs = require("fs");
const rawPath = "./raw"
const savePath = "./save"

const matchSep = path.sep === "\\" ? "\\\\" : "\/"

const language = ["zh_cn", "default"]
const jsonArray = [
    "loot.json",
    // "profile-icons.json",
    "summoner-emotes.json",
    "summoner-icons.json",
    "summoner-icon-sets.json",
    "ward-skins.json",
    "ward-skin-sets.json",
]

const url = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/{language}/v1/"
const profileUrlCn = "https://dlied1.qq.com/lolapp/lol/summoner/profileicon/"
const verUrl = "https://ddragon.leagueoflegends.com/realms/tencent.json"

function clearCache() {
    if (fs.existsSync(path.resolve(rawPath))) {
        fs.rmSync(rawPath, { recursive: true });
    }
    if (fs.existsSync(path.resolve(savePath))) {
        fs.rmSync(savePath, { recursive: true });
    }
}

async function retrieveRaw() {
    for (let lang of language) {
        if (!fs.existsSync(path.resolve(rawPath, lang))) {
            fs.mkdirSync(path.resolve(rawPath, lang), { recursive: true })
        }
        for (let fileName of jsonArray) {
            let mypath = path.resolve(rawPath, lang, fileName)
            const writer = fs.createWriteStream(mypath)
            let response = await axios({
                url: url.replace('{language}', lang) + fileName,
                method: "GET",
                responseType: "stream",
            })
            console.log("fetch: ", url.replace('{language}', lang) + fileName)
            response.data.pipe(writer)
            new Promise((resolve, reject) => {
                writer.on("finish", resolve)
                writer.on("error", reject)
            });
        }
    }
}


function parseLoot() {
    for (let lang of language) {
        let lootPath = path.resolve(rawPath, lang, 'loot.json')
        if (!fs.existsSync(lootPath)) {
            throw new Error(`没有找到${lootPath}`)
        }
        let newJson = []
        let rawJson = JSON.parse(fs.readFileSync(lootPath, 'utf-8').toString())

        for (const rawJsonElement of rawJson.LootItems) {
            if (rawJsonElement.id.match(/CHEST_[0-9]{1,4}$/g) != null) {
                newJson.push(rawJsonElement)
            }
        }
        newJson.sort((a, b) => {
            return a.mappedStoreId - b.mappedStoreId
        })
        if (newJson.length !== 0) {
            fs.writeFileSync(lootPath, JSON.stringify(newJson))
        }
    }
}

const walkSync = (dir, callback) => {
    const files = fs.readdirSync(dir);
    files.forEach((file) => {
        var filepath = path.join(dir, file);
        const stats = fs.statSync(filepath);
        if (stats.isDirectory()) {
            walkSync(filepath, callback);
        } else if (stats.isFile()) {
            callback(filepath, stats);
        }
    });
};

function replaceAssetsPath(filepath, stats) {
    fs.readFile(filepath, 'utf-8', (err, data) => {
        if (err) {
            console.log(err);
        } else {
            let re = new RegExp(`${matchSep}([a-z_]{1,8})${matchSep}(.*)`)
            let mat = filepath.match(re)
            let lang = mat[1]  // 最好用default
            let filename = mat[2]
            let parsedData;
            switch (filename) {
                case 'summoner-icons.json':
                    if (lang === 'zh_cn') {
                        parsedData = data.replace(/\/lol-game-data\/assets\/v1\/profile-icons\/([0-9]+\.(jpg|png))/g, (match, p1) => {
                            return `${profileUrlCn}${p1.toLowerCase()}`
                        })
                        break;
                    }
                default:
                    parsedData = data.replace(/\/lol-game-data\/assets\/((v1|content|ASSETS)[A-Za-z0-9\/.\-_]+\.(png|jpg))/g, (match, p1, p2, p3, offset, string) => {
                        // 最好lang用default
                        return `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/${p1.toLowerCase()}`
                    })
            }
            if (parsedData.length !== 0) {
                if (!fs.existsSync(path.resolve(savePath, lang))) {
                    fs.mkdirSync(path.resolve(savePath, lang), { recursive: true })
                }
                fs.writeFileSync(path.resolve(savePath, lang, filename), parsedData)
                console.log(`save: ${path.resolve(savePath, lang, filename)}`)
            }

        }
    })
}

async function genPackage() {
    let verJson = await axios.get(verUrl)
    let package = `{
  "name": "@magicwenli/league-fan-assets",
  "version": "${verJson.data.v}-v${Date.now().toString()}",
  "sourceVersion": "${verJson.data.v}",
  "description": "league-fan assets.",
  "main": "index.js",
  "author": "magicwenli",
  "license": "MIT"
}`
    fs.writeFileSync(path.resolve(savePath, 'package.json'), package)
    fs.writeFileSync(path.resolve(savePath, 'version.json'), JSON.stringify(verJson.data, null, 2))
}


clearCache()
retrieveRaw().then(res => {
    parseLoot()
    walkSync(rawPath, replaceAssetsPath)
    genPackage().then(res => {
        console.log(`write: ${path.resolve(savePath, 'package.json')}`)
    })
})
