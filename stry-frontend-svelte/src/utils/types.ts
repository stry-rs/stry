export enum Contrast {
	High,
	Low,
	Disabled,
}

export enum Rating {
	Explicit,
	Mature,
	Teen,
	General,
}

export enum State {
	Completed,
	InProgress,
	Hiatus,
	Abandoned,
}

export interface IStory {
	id: string;

	name: string,
	summary: IPart[],

	rating: Rating,
	state: State,

	authors: IUser[],
	commissioners: IUser[],
	dedicatees: IUser[],

	origins: IOrigin[],

	warnings: IWarning[],
	pairings: IPairing[],
	characters: ICharacter[],
	tags: ITag[],
}

export interface IPart {
	id: string;
	kind: PartKind;
	content: string;
}

export enum PartKind {
	Paragraph,
}

export interface IUser {
	id: string;
	name: string;
}

export interface IOrigin {
	id: string;
	text: string;
}

export interface IWarning {
	id: string;
	text: string;
}

export interface IPairing {
	id: string;
	text: string;
	minor: boolean;
}

export interface ICharacter {
	id: string;
	text: string;
	minor: boolean;
}

export interface ITag {
	id: string;
	text: string;
}

export enum TagKind {
	Warning,
	Pairing,
	PairingMinor,
	Character,
	CharacterMinor,
	General,
}

export class TagKindUtil {
	public static isMinor(kind: TagKind): boolean {
		switch (kind) {
			case TagKind.PairingMinor:
			case TagKind.CharacterMinor:
				return true;
			default:
				return false;
		}
	}

	public static asPathPrefix(kind: TagKind): string {
		switch (kind) {
			case TagKind.Warning:
				return "/warning/";
			case TagKind.Pairing:
			case TagKind.PairingMinor:
				return "/pairing/";
			case TagKind.Character:
			case TagKind.CharacterMinor:
				return "/character/";
			case TagKind.General:
				return "/tag/";
			default:
				return "/";
		}
	}
}
