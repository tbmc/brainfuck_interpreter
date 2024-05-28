import { spawn } from 'child_process';
import { WebSocketServer, type WebSocket } from 'ws';
import type {
    WebSocketFirstMessage,
    WebSocketInputMessage,
    WebSocketOtherInputMessages,
    WebSocketOutputMessages,
} from '#shared/websocketMessage';
import type { ChildProcessWithoutNullStreams } from 'node:child_process';
import tmp from 'tmp';
import fs from 'fs';

function getExecutable(): string {
    return process.platform === 'win32'
        ? 'brain_fuck_interpreter.exe'
        : 'brain_fuck_interpreter';
}

const wss = new WebSocketServer({ port: 3001 });

wss.on('connection', (ws: WebSocket) => {
    const tmpObj = tmp.fileSync();
    let command: ChildProcessWithoutNullStreams | null = null;
    const handleFirstMessage = (ws: WebSocket, code: string) => {
        fs.writeFileSync(tmpObj.name, code);
        fs.writeFileSync('./tmp.bf', code);

        command = spawn(getExecutable(), [tmpObj.name], {
            cwd: '..',
        });
        command.stdout.on('data', (data) => {
            const toSend: WebSocketOutputMessages = {
                output: data.toString(),
                closed: false,
            };
            ws.send(JSON.stringify(toSend));
            // process.stdout.write(data.toString());
        });
        command.stderr.on('data', (data) => {
            // console.error('Error:', data.toString());
        });
        command.on('close', () => {
            const toSend: WebSocketOutputMessages = {
                output: '',
                closed: true,
            };
            ws.send(JSON.stringify(toSend));
            tmpObj.removeCallback();
        });
    };
    const handleNextMessages = (input: string) => {
        if (command === null) throw new Error('Should not happen');

        command.stdin.write(input);
    };

    ws.on('open', () => {
        console.info('New connection');
    });

    ws.on('message', (message: Buffer) => {
        const data = JSON.parse(message.toString()) as WebSocketInputMessage;
        // console.debug('New message', data);
        if (data.isFirstMessage)
            handleFirstMessage(ws, (data as WebSocketFirstMessage).code);
        else handleNextMessages((data as WebSocketOtherInputMessages).input);
    });
});

console.log('Listening WebSocket on port 3001');
