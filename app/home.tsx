import { Button, Image, Pressable, ScrollView, StyleSheet, Text, View } from "react-native";
import { DefaultContainer, colorScheme, defaultFont, defaultFontBold, defaultFontItalic, fontSize } from "../constants/style";
import { useEffect } from "react";

function Home({navigation, route}: {navigation: any, route: any}) {
    const username = route.params.username;
    const password = route.params.password;

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

    return <DefaultContainer menu="home">
        <ScrollView contentInsetAdjustmentBehavior='automatic' style={{
            backgroundColor: colorScheme.background,
            flex: 1,
            padding: 20,
            paddingTop: 10,
        }}>
            <Text style={{
                fontFamily:defaultFont,
                fontSize: fontSize.small,
                color:colorScheme.primary,
            }} >
                good evening, {username}
            </Text>
        </ScrollView>
    </DefaultContainer>
}

export default Home;