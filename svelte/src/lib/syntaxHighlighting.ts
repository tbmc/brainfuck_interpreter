// todo: defer syntax highlight
export const highlight = (text: string): string => {
	return text
		.replace('+', '<span class="plus-color">+</span>')
		.replace('-', '<span class="minus-color">-</span>')
		.replace('&lsaquo;', '<span class="chevron-color">&lsaquo;</span>')
		.replace('&rsaquo;', '<span class="chevron-color">&rsaquo;</span>')
		.replace('[', '<span class="bracket-color">[</span>')
		.replace(']', '<span class="bracket-color">]</span>')
		.replace(',', '<span class="comma-color">,</span>')
		.replace('.', '<span class="dot-color">,</span>');
};
