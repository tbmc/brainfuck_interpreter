import { type WebSocket, WebSocketServer } from 'ws';
import tmp from 'tmp';
import type { ChildProcessWithoutNullStreams } from 'node:child_process';
import fs from 'fs';
import { spawn } from 'child_process';
import type {
    WebSocketFirstMessage,
    WebSocketInputMessage,
    WebSocketOtherInputMessages,
    WebSocketOutputMessages,
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-expect-error
} from '#shared/websocketMessage';
import { server } from './proxy';

function getExecutable(): string {
    return 'brainfuck_interpreter';
}

export function runWebSocketServer() {
    const wss = new WebSocketServer({ server });

    wss.on('connection', (ws: WebSocket) => {
        try {
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
                const data = JSON.parse(
                    message.toString()
                ) as WebSocketInputMessage;
                // console.debug('New message', data);
                if (data.isFirstMessage)
                    handleFirstMessage(
                        ws,
                        (data as WebSocketFirstMessage).code
                    );
                else
                    handleNextMessages(
                        (data as WebSocketOtherInputMessages).input
                    );
            });
        } catch (err) {
            console.error(err);
        }
    });
}
