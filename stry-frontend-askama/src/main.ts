// https://github.com/selfawarestudio/marth
namespace utils {
	type EventMap<T extends EventTarget> = T extends MediaQueryList ? MediaQueryListEventMap : T extends Document  ? DocumentEventMap  : T extends Window ? WindowEventMap : HTMLElementEventMap;

	type EventTypes<T extends EventTarget> = keyof EventMap<T> & string;

	type EventValue<T extends EventTarget, K extends EventTypes<T>> = Extract<EventMap<T>[K], Event>;

	let call = <T, U>(xs: T | T[], fn: (x: T, i: number) => U): U[] => ([] as T[]).concat(xs).map(fn);

	let classes = (action: 'add' | 'remove'): ((xs: Element | Element[], ...cn: string[]) => any[]) => (xs, cns) => call(xs, x => x.classList[action](...cns.split(' ')));

	export let id = (selector: string) => document.getElementById(selector);

	export let add = (xs: Element | Element[], cns: string): void => {
		classes('add')(xs, cns);
	}

	export let remove = (xs: Element | Element[], cns: string): void => {
		classes('remove')(xs, cns);
	}

	export let toggle = (
		xs: Element | Element[],
		cn: string,
		force?: boolean,
	): void => {
		call(xs, x => x.classList.toggle(cn, force));
	}

	let events = (action: 'add' | 'remove') => <T extends EventTarget, K extends EventTypes<T>>(xs: T | T[], t: K, fn: (ev: EventValue<T, K>) => void, opts?: boolean | AddEventListenerOptions) => call(xs, x => x[`${action}EventListener`](t, fn as EventListener, opts));

	export let on = <T extends EventTarget, K extends EventTypes<T>>(xs: T | T[], t: K, fn: (ev: EventValue<T, K>) => void, opts?: boolean | AddEventListenerOptions) => {
		events('add')(xs, t, fn, opts)
		return () => events('remove')(xs, t, fn, opts)
	}
}

interface State {
	navbarOpen: boolean;
}

declare var __STATE__: State;
window.__STATE__ = {
	navbarOpen: false,
};

(() => {
	const navbarButton = utils.id("navbar-button");
	const navbarButtonIconOpen = utils.id("navbar-button__icon--open");
	const navbarButtonIconClose = utils.id("navbar-button__icon--close");
	if (navbarButton != null && navbarButtonIconOpen != null && navbarButtonIconClose != null) {
		utils.on(navbarButton, "click", () => {
			window.__STATE__.navbarOpen = !window.__STATE__.navbarOpen;

			utils.toggle(navbarButtonIconOpen, "block");
			utils.toggle(navbarButtonIconOpen, "hidden");
			utils.toggle(navbarButtonIconClose, "block");
			utils.toggle(navbarButtonIconClose, "hidden");
		});
	} else {
		console.error("required navbar elements are missing");
	}
})();
