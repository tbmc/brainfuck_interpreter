import fs from 'fs/promises';
import path from 'path';

const folderPath = path.resolve(__dirname, '..', 'brain_fuck_scripts');

async function listFiles(): Promise<string[]> {
	const files = await fs.readdir(folderPath);
	const fileList = [];

	for (const file of files) {
		const filePath = path.join(folderPath, file);
		const fileStat = await fs.stat(filePath);

		if (fileStat.isFile()) fileList.push(file);
	}

	return fileList;
}

function removeExtension(fileName: string): string {
	return fileName.replace('.bf', '');
}

function sanitize(text: string): string {
	return text.replace('`', '\\`');
}

async function filesTo1File() {
	const files = await listFiles();
	let text = 'const scripts: Record<string, string> = {\n';

	for (const file of files) {
		const fileContent = await fs.readFile(path.join(folderPath, file), 'utf-8');
		text += `${removeExtension(file)}: \`\n${sanitize(fileContent)}\`,\n`;
	}

	text += '};\n\n';
	text += 'export default scripts;\n';

	await fs.writeFile(path.join(__dirname, 'src', 'lib', 'brain_fuck_scripts.ts'), text);
}

export const generateBrainFuckFiles = () => ({
	name: 'plugin generate brainfuck files',
	buildStart: async () => {
		console.log('Generate brainfuck files!');
		await filesTo1File();
	}
});
