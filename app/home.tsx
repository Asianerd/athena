import { Button, Image, Pressable, ScrollView, StyleSheet, Text, View } from "react-native";
import { DefaultContainer, colorScheme, defaultFont, defaultFontBold, defaultFontItalic, fontSize } from "../constants/style";
import React, { useEffect } from "react";

import GLOBALS from './global';
import { screenSize } from "./_layout";
import { Task } from "@/constants/Task";
import { DrawerActions } from "@react-navigation/native";

function HyperlinkHeader({header}: {header: any}) {
    return (
        <View style={{
            marginTop: 10,
            marginBottom: 10
        }}>
            <Text style={{ color:colorScheme.primary, fontFamily: defaultFontItalic, fontSize:fontSize.tiny * 1.3 }}>
                {header}
            </Text>
        </View>
    );
}

async function fetchTasks(rootTask: Task | undefined, changeRootTask: any) {
    if (GLOBALS.selectedProject != undefined) {
        changeRootTask(await Task.fetchTasks(GLOBALS.selectedProject['id']));
    }
}

function Home({navigation, route}: {navigation: any, route: any}) {
    let [rootTask, changeRootTask] = React.useState<Task | undefined>(undefined);

    useEffect(() => {
        fetchTasks(rootTask, changeRootTask);
    }, [GLOBALS.selectedProject])

    return <DefaultContainer menu="home">
        <View style={{
            position:'absolute',
            zIndex:1000
        }}>
            <Pressable onPress={() => { navigation.openDrawer(); }} style={{
                backgroundColor:'#0007',
                borderRadius:10,
                marginLeft:10,
                marginTop:10
            }}>
                <Image source={require('../assets/burger.png')} height={100} width={100} style={{
                    width:25,
                    height:25,
                    margin:12,
                }} />
            </Pressable>
        </View>
        {
            GLOBALS.selectedProject == undefined ?
            <View style={{
                height:'100%',
                display:'flex',
                justifyContent:'center',
                alignContent:'center'
            }}>
                <Text style={{
                    fontSize:fontSize.small,
                    fontFamily:defaultFontBold,
                    color:colorScheme.primary,
                    opacity:0.5,
                    textAlign:'center'
                }}>
                    no project selected
                </Text>
                <Text style={{
                    fontSize:fontSize.tiny,
                    fontFamily:defaultFontItalic,
                    color:colorScheme.primary,
                    opacity:0.3,
                    textAlign:'center',
                    marginTop:15
                }}>
                    select a project to get started
                </Text>
            </View>
            : 
            <ScrollView contentInsetAdjustmentBehavior='automatic' style={{
                backgroundColor: colorScheme.background,
                flex: 1,
                paddingTop: 10,
            }}>
                <ScrollView horizontal>
                    <View style={{
                        display:'flex',
                        justifyContent:'flex-start',
                        alignItems:'center',
                        flexDirection:'column',

                        minHeight:screenSize.height,
                        minWidth:screenSize.width
                    }}>
                        <View style={{
                            borderStyle:'solid',
                            borderBottomColor:'#fff',
                            borderBottomWidth:2,
                        }}>
                            <Text style={{
                                fontSize:fontSize.small,
                                fontFamily:defaultFontBold,
                                color:colorScheme.primary,
                                width:'auto',
                            }}>
                                {GLOBALS.selectedProject['title']}
                            </Text>
                        </View>
                        {
                            rootTask == undefined ? 
                            <View>

                            </View>
                            :
                            <View style={{
                                marginTop:20
                            }}>
                                { Task.Item(rootTask, 0) }
                            </View>
                        }
                    </View>
                </ScrollView>
            </ScrollView>
        }
    </DefaultContainer>
}

export default Home;