export interface Destroyable {
	destroy(): void
}

export default function clickOutside(node: HTMLElement, onEventFunction: () => void): Destroyable {
	const handleClick = (event: MouseEvent) => {
		const path = event.composedPath();

		if (!path.includes(node)) {
			onEventFunction();
		}
	};

	document.addEventListener("click", handleClick);

	return {
		destroy() {
			document.removeEventListener("click", handleClick);
		}
	}
}