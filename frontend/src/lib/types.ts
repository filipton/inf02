export type Question = {
    id: number;
    text: string;
    image: string | null;
    anwsers: string[];
    correct: number;
    selected: number; // NOT IN JSON
    starred: boolean; // NOT IN JSON
};

export type SavedState = {
    questions: Question[];
    startedAt: number;
}

export type DocsEntry = {
    url: string;
    markdown: string;
};
