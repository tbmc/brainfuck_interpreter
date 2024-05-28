export interface WebSocketInputMessage {
	isFirstMessage: boolean;
}

export interface WebSocketFirstMessage extends WebSocketInputMessage {
	code: string;
	isFirstMessage: true;
}

export interface WebSocketOtherInputMessages extends WebSocketInputMessage {
	input: string;
	isFirstMessage: false;
}

export interface WebSocketOutputMessages {
	output: string;
	closed: boolean;
}
