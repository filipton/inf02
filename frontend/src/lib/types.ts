export type Question = {
    text: string;
    image: string | null;
    anwsers: string[];
    correct: number;
    selected: number; // NOT IN JSON
};
