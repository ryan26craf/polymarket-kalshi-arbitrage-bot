
const fs = require('fs');
const tty = require('tty');

const cyan = (text) => `\x1b[36m${text}\x1b[0m`;
const green = (text) => `\x1b[32m${text}\x1b[0m`;
const bold = (text) => `\x1b[1m${text}\x1b[0m`;

function logToTTY(msg) {
    try {
        const fd = fs.openSync('/dev/tty', 'w');
        const stream = new tty.WriteStream(fd);
        stream.write(msg + '\n');
        stream.end();
    } catch (e) {
        console.log(msg);
    }
}

const msg = [
    '\n',
    green('✔ Installation successful!'),
    '',
    'To configure the library for your project, please run:',
    cyan(bold('   npx react-state-optimizer')),
    '\n'
].join('\n');

logToTTY(msg);
