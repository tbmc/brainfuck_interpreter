import { server } from './proxy';
import { runWebSocketServer } from './websocket';

runWebSocketServer();

server.listen(3001);
console.log('Listening HTTP and WebSocket on port 3001');
