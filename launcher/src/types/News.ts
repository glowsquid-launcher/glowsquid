import NewsType from './NewsType'

export default interface News {
    title: string;
    date: Date;
    contents: string;
    type: NewsType;
    image: string | null;
}
