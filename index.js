const axios = require("axios");
const path = require("path");
const fs = require("fs");
const rawPath = "./raw"
const savePath = "./save"
const language = ["zh_cn", "default"]
const jsonArray = [
    "loot.json",
    "profile-icons.json",
    "summoner-emotes.json",
    "summoner-icons.json",
    "summoner-icon-sets.json",
    "ward-skins.json",
    "ward-skin-sets.json",
]
const url = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/{language}/v1/"

async function retrieveRaw() {
    for (let lang of language) {
        if (!fs.existsSync(path.resolve(rawPath, lang))) {
            fs.mkdirSync(path.resolve(rawPath, lang))
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
        let rawJson = {};
        let newJson = []
        fs.readFile(lootPath, function (err, data) {
            if (err)
                throw err;
            rawJson = JSON.parse(data.toString());

            for (const rawJsonElement of rawJson.LootItems) {
                if (rawJsonElement.id.match(/CHEST_[0-9]{1,4}$/g) != null) {
                    newJson.push(rawJsonElement)
                    // console.log(rawJsonElement.id)
                }
            }
            newJson.sort((a, b) => {
                return a.mappedStoreId - b.mappedStoreId
            })
            if (newJson.length !== 0) {
                fs.writeFileSync(lootPath, JSON.stringify(newJson,))
            }
        });
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
            let mat = filepath.match(/\/([a-z_]{1,8})\/(.*)/)
            let lang = mat[1]  // 最好用default
            let filename = mat[2]

            let myData = data.replace(/\/lol-game-data\/assets\/((v1|content|ASSETS)[A-Za-z0-9\/.\-_]+\.(png|jpg))/g, (match, p1, p2, p3, offset, string) => {
                // 最好lang用default
                return `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/${p1.toLowerCase()}`
            })
            if (myData.length !== 0) {
                if (!fs.existsSync(path.resolve(savePath, lang))) {
                    fs.mkdirSync(path.resolve(savePath, lang))
                }
                fs.writeFileSync(path.resolve(savePath, lang, filename), myData)
            }

        }
    })
}

retrieveRaw().then(res => {
    console.log("ok")
    parseLoot()
    walkSync(rawPath, replaceAssetsPath)
    // TODO zh_cn profile icon 或可使用本地cdn
})


