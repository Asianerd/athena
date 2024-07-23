import { colorScheme, defaultFont, defaultFontBold, fontSize } from "@/constants/style";
import { useState } from "react";
import { Image, Pressable, Text, View } from "react-native";

// let selected = 0;
// const [selection, onSelectionChange] = useState(0);

// let projectData = [
//     {
//         id: 12345,
//         title: 'lorem ipsum'
//     },
//     {
//         id: 23456,
//         title: 'dolor sit amet'
//     },
// ];



// function AllDrawerItems() {
//     let result = [];
//     projectData.forEach((e) => {
//         result.push(<DrawerItem data={e} />)
//     })
//     // console.log(result);
//     return (<View style={{
//         backgroundColor:'red',
//         flex:1
//     }}>
//         {/* result.map((d) => d) */}
//     </View>);
// }

export function CustomDrawerContent(safeAreaInsets: any) {
    const [selection, changeSelection] = useState(0);

    const [projectData, changeProjectData] = useState([
        {
            id: 12345,
            title: 'lorem ipsum'
        },
        {
            id: 23456,
            title: 'dolor sit amet'
        },
    ]);

    // let projectData = [
    //     {
    //         id: 12345,
    //         title: 'lorem ipsum'
    //     },
    //     {
    //         id: 23456,
    //         title: 'dolor sit amet'
    //     },
    // ];

    function DrawerItem({data} : { data:any }) {
        console.log()
        return (
            <Pressable onPress={() => { changeSelection(data.id); console.log(`open project : ${data.id}`) }} style={{
                backgroundColor: selection == data.id ? colorScheme.border : '#0000',
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
                {/* <View style={{
                    backgroundColor:'blue',
                    height:40,
                    width:'100%',
                    marginBottom:10,
                    justifyContent:'center',
                    alignItems:'center'
                }}>
                </View> */}
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
                // backgroundColor:'red',
                // height:200,
                flex:1,
                justifyContent:'center',
                // alignItems:'center',
                flexDirection:'column'
            }}>
                <DrawerItem data={projectData[0]}/>
                <DrawerItem data={projectData[1]}/>
                {/* <AllDrawerItems/> */}
                {
                    // let result = [];
                    // projectData.forEach((e) => {
                    //     <DrawerItem data={e} />;
                    // })
                }
                {/* <DrawerItem data={} /> */}
            </View>
            <View style={{
                // backgroundColor:'blue',
                // height:200,
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
                        marginRight:10
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