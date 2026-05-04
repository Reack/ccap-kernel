const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

const version = "0.1.4";
const repo = "Reack/ccap-kernel";

const platform = process.platform;
const arch = process.arch;

let binName = '';
if (platform === 'win32' && arch === 'x64') {
    binName = 'ccap-windows-x64.exe';
} else if (platform === 'darwin' && arch === 'x64') {
    binName = 'ccap-macos-x64';
} else if (platform === 'linux' && arch === 'x64') {
    binName = 'ccap-linux-x64';
}

if (!binName) {
    console.error(`❌ Unsupported platform: ${platform} ${arch}`);
    process.exit(1);
}

const url = `https://github.com/${repo}/releases/download/v${version}/${binName}`;
const binDir = path.join(__dirname, '..', 'binaries');
const binPath = path.join(binDir, binName);

if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
}

console.log(`🚀 Downloading CCAP binary for ${platform} from ${url}...`);

const file = fs.createWriteStream(binPath);
https.get(url, (response) => {
    if (response.statusCode === 302 || response.statusCode === 301) {
        https.get(response.headers.location, (res) => {
            res.pipe(file);
            file.on('finish', () => {
                file.close();
                if (platform !== 'win32') {
                    fs.chmodSync(binPath, '755');
                }
                console.log('✅ Binary downloaded successfully.');
            });
        });
    } else if (response.statusCode === 200) {
        response.pipe(file);
        file.on('finish', () => {
            file.close();
            if (platform !== 'win32') {
                fs.chmodSync(binPath, '755');
            }
            console.log('✅ Binary downloaded successfully.');
        });
    } else {
        console.error(`❌ Download failed. Status code: ${response.statusCode}`);
        process.exit(1);
    }
}).on('error', (err) => {
    fs.unlink(binPath, () => {});
    console.error(`❌ Error downloading binary: ${err.message}`);
    process.exit(1);
});
