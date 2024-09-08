import { fetchProjects } from "@/app/_layout";
import { Project } from "@/constants/Project";
import { colorScheme, defaultFont, defaultFontBold, defaultFontItalic, fontSize } from "@/constants/style";
import React from "react";
import { Image, Pressable, Text, View } from "react-native";

export function CustomDrawerContent(safeAreaInsets: any, selection: Project | undefined, changeSelection: any, projectList: {[id: string]: Project}, changeProjectList: any) {
    function DrawerItem({data} : { data: any }): React.JSX.Element {
        return (
            <Pressable key={data.id} onPress={() => { changeSelection(data); console.log(`open project : ${data.id}`); }} style={{
                backgroundColor: selection?.id == data.id ? colorScheme.border : '#0000',
                height:40,
                width:'100%',
                marginBottom:10,
                justifyContent:'center',
                alignItems:'center',
                borderRadius:10
            }}>
                <Text style={{
                    fontSize:fontSize.small,
                    fontFamily:defaultFontBold,
                    color:colorScheme.primary,
                }}>
                    {data.title}
                </Text>
            </Pressable>
        );
    }

    return (
        <View style={{
            backgroundColor:colorScheme.secondary,
            height:'100%',
            paddingTop:safeAreaInsets.top,
            paddingBottom:safeAreaInsets.bottom,
            paddingHorizontal: 20
        }}>
            <View style={{
                flex:1,
                justifyContent:'center',
                flexDirection:'column'
            }}>
                {
                    Object.entries(projectList).map(([k, v]) => {
                        return (
                            <DrawerItem data={v} />
                        )
                    })
                }
                <Pressable onPress={ async () => { await fetchProjects(changeProjectList); }} style={{
                    marginTop:15,
                    width:'100%',
                }}>
                    <Text style={{
                        fontFamily:defaultFontItalic,
                        color:'#fff8',
                        textAlign:'center',
                        textDecorationStyle:'solid',
                        textDecorationColor:'#fff8'
                    }}>
                        refresh
                    </Text>
                </Pressable>
            </View>
            <View style={{
                marginTop:20,
                justifyContent:'space-between',
                flexDirection:'row'
            }}>
                <View style={{
                    display:'flex',
                    flexDirection:'row',
                    alignItems:'center',
                }}>
                    <Image source={require('../assets/profile.png')} style={{
                        // height: 50,
                        width: 25,
                        aspectRatio:1,
                        marginRight:15
                    }}/>
                    <Text style={{
                        fontSize:fontSize.small,
                        fontFamily:defaultFont,
                        color: colorScheme.primary
                    }}>han_yuji_</Text>
                </View>
                <Pressable style={{
                    width:25,
                    aspectRatio:1
                }}>
                    <Image source={require('../assets/settings.png')} style={{
                        height:'100%',
                        width:'100%'
                    }}/>
                </Pressable>
            </View>
        </View>
    );
}