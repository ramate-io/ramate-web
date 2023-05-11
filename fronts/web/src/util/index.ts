export interface ShortDisplayCharacter {
    displayText : string;
    color : string;
    selected : boolean;
}

export interface TitleSubtitleDisplay {
    title : string;
    subtitle : string;
}

export interface ExpandableSectionDisplay extends TitleSubtitleDisplay {
    expanded : boolean;
}