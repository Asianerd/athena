import React from 'react';
import GLOBALS from '../app/global';
import { Pressable, Text, View } from 'react-native';
import { colorScheme, defaultFont, fontSize } from './style';

const pointerWidth = 2;
const pointerRadius = 10;

export class Task {
    id: number;
    title: string;
    description: string;
    children: Array<Task>;
    parent: number;

    constructor(id: number, title: string, description: string, parent: number) {
        this.id = id;
        this.title = title;
        this.description = description;
        this.parent = parent;

        this.children = [];
    }

    public static parse(r: any): Task {
        return new Task(
            r['id'],
            r['title'],
            r['description'],
            r['parent']
        );
    }

    public static fetchFormatted(collection: { [id: number]: Task }): Task | undefined {
        // returns root task that contains all its children
        let root = this.fetchRoot(collection);

        if (root === undefined) { 
            return undefined;
        }

        root.fetchChildrenRecursive(collection);

        return root;
    }

    public static parseCollection(collection: any): { [id:number]: Task } {
        let result: { [id: number]: Task } = {};
        for (let key in collection) {
            result[parseInt(key)] = Task.parse(collection[key]);
        }

        return result;
    }

    public static fetchRoot(collection: {[id: number]: Task}): Task | undefined {
        // fetches the root task, the one to start it all (does not have a parent)
        let candidates = Object.entries(collection).filter((e) => { return e[1].parent == -1 });
        return candidates.length <= 0 ? undefined : candidates[0][1];
    }

    public fetchChildren(collection: { [id: number]: Task }) {
        // initializes children of task
        this.children = Object.entries(collection).filter((e) => { return e[1].parent == this.id }).map((e) => { return e[1] });
    }

    public fetchChildrenRecursive(collection: { [id: number]: Task }) {
        this.fetchChildren(collection);
        this.children.forEach((c) => {
            c.fetchChildrenRecursive(collection);
        });
    }

    public static async fetchTasks(project_id: number): Promise<Task | undefined> {
        try {
            let result = await fetch(`${GLOBALS.backendAddress}/task/fetch/${project_id}`, {
                method: 'POST',
                headers: {
                    'Content-type': 'text/plain'
                },
                body: JSON.stringify({
                    username:GLOBALS.username,
                    password:GLOBALS.password
                })
            });
            let json = await result.json();
            if (JSON.parse(decodeURIComponent(json['data'])) == 'not a member of project') {
                return undefined;
            }
            return Task.fetchFormatted(Task.parseCollection(JSON.parse(decodeURIComponent(json['data']))));
        } catch (e) {
            console.log(`fetchTasks() -> error found : ${e}`);
        }
    }

    public static PointerItem(t: number): React.JSX.Element {
        return t == 0 ?
        // connect to right
        <View style={{
            display:'flex',
            justifyContent:'flex-end',
            alignItems:'flex-end',
            width:'100%',
            height:'100%'
        }}>
            <View style={{
                width:'50%',
                height:'100%',
                borderTopLeftRadius:pointerRadius,
                borderStyle:'solid',
                borderTopWidth:pointerWidth,
                borderLeftWidth:pointerWidth,
                borderColor:colorScheme.border
            }}/>
        </View>
        : t == 1 ?
        // left, right
        <View style={{
            display:'flex',
            justifyContent:'flex-end',
            alignItems:'flex-end',
            width:'100%',
            height:'100%'
        }}>
            <View style={{
                width:'100%',
                height:'100%',
                borderStyle:'solid',
                borderTopWidth:pointerWidth,
                borderColor:colorScheme.border
            }}>
                <View style={{
                    backgroundColor:colorScheme.border,
                    width:pointerWidth,
                    height:'100%',
                    alignSelf:'center'
                }} />
            </View>
        </View>
        :
        // connect to left
        <View style={{
            display:'flex',
            justifyContent:'flex-end',
            alignItems:'flex-start',
            width:'100%',
            height:'100%'
        }}>
            <View style={{
                width:'50%',
                height:'100%',
                borderTopRightRadius:pointerRadius,
                borderStyle:'solid',
                borderTopWidth:pointerWidth,
                borderRightWidth:pointerWidth,
                borderColor:colorScheme.border
            }}/>
        </View>
    }

    public static Item(data: Task, t: number): React.JSX.Element {
        return (
            <Pressable onPress={() => {
                console.log(`${data.title} pressed`);
            }}>
                <View key={data.id}>
                    {
                        (data.parent != -1) && <View style={{
                            width:'100%',
                            height:20,
                        }}>
                            { Task.PointerItem(t) }
                        </View>
                    }
                    <View style={{

                    }}>
                        <View style={{
                            display:'flex',
                            justifyContent:'center',
                            alignItems:'center',
                            margin:10,
                            marginTop:0,
                            marginBottom:0
                        }}>
                            <Text style={{
                                fontFamily:defaultFont,
                                fontSize:fontSize.small,
                                color:colorScheme.primary,
                                borderStyle:'solid',
                                borderColor:colorScheme.border,
                                borderWidth:2,
                                padding:10,
                                borderRadius:5
                            }}>
                                { data.title }
                            </Text>
                        </View>
                        {
                            data.children.length > 0 && <View style={{
                                backgroundColor:colorScheme.border,
                                height:20,
                                width:pointerWidth,
                                alignSelf:'center'
                            }} />
                        }
                    </View>
                    <View style={{
                        display:'flex',
                        justifyContent:'center',
                        alignItems:'flex-start',
                        flexDirection:'row',
                    }}>
                        {
                            data.children.map((t, i) => Task.Item(t, i == 0 ? 0 : (i == (data.children.length - 1) ? 2 : 1)))
                        }
                    </View>
                </View>
            </Pressable>
        )
    }
}
