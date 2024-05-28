import type {
	WebSocketFirstMessage,
	WebSocketOtherInputMessages,
	WebSocketOutputMessages
} from '$lib/shared/websocketMessage';

let socket: WebSocket | null = null;

export const connectWebsocket = (
	code: string,
	onChange: (data: string) => void,
	onClose: () => void
): ((data: string) => void) => {
	if (socket !== null) {
		socket.close();
	}

	const onOpen = () => {
		if (socket === null) throw new Error('This should never happen');
		console.log('Websocket opened');

		socket.send(
			JSON.stringify({
				code,
				isFirstMessage: true
			} as WebSocketFirstMessage)
		);
	};

	const onMessage = (message: MessageEvent) => {
		if (socket === null) return;
		const data: WebSocketOutputMessages = JSON.parse(message.data);
		if (data.closed) {
			onClose();
		} else {
			onChange(data.output);
		}
	};

	const protocol = window.location.protocol;
	const websocketProtocol = protocol.includes('https') ? 'wss' : 'ws';
	const host = window.location.host;
	socket = new WebSocket(`${websocketProtocol}://${host}/`);
	socket.addEventListener('open', onOpen);
	socket.addEventListener('message', onMessage);

	return (data: string) => {
		if (socket === null) return null;

		const dataToSend: WebSocketOtherInputMessages = {
			isFirstMessage: false,
			input: data
		};
		socket.send(JSON.stringify(dataToSend));
	};
};
