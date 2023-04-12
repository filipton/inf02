export type Question = {
    id: number;
    text: string;
    image: string | null;
    anwsers: string[];
    correct: number;
    selected: number; // NOT IN JSON
};
