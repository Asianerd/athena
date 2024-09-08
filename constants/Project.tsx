import GLOBALS from '../app/global';

export class Project {
    id: number;
    title: string;
    description: string;

    constructor(id: number, title: string, description: string) {
        this.id = id;
        this.title = title;
        this.description = description;
    }

    static parse(t: any): Project {
        return new Project(
            t['id'],
            t['title'],
            t['description'],
        );
    }

    static parseCollection(collection: any): { [id: number]: Project } {
        let result: { [id: number]: Project } = {};
        for (let key in collection) {
            result[parseInt(key)] =  Project.parse(collection[key]);
        }

        return result;
    }

    public static async fetchProjects(): Promise<{ [id: number]: Project }> {
        let result = await fetch(`${GLOBALS.backendAddress}/project/fetch/owned`, {
            method: 'POST',
            body: JSON.stringify({
                username: GLOBALS.username,
                password: GLOBALS.password
            })
        });
        let json = await result.json();

        return Project.parseCollection(JSON.parse(decodeURIComponent(json['data'])));
    }
};