import { Image, Pressable, StyleSheet, View } from "react-native";
import { colorScheme } from "../constants/style";
import React from "react";

const navbarStyles = StyleSheet.create({
    icon: {
        width:40,
        aspectRatio:1,
        objectFit: 'contain',
        // backgroundColor:'red',
        display:'flex',
        justifyContent:'flex-end',
        alignItems:'center'
    }
})

const iconPaths: { [key: string]: any } = {
    'home': require('../assets/home.png'),
    'search': require('../assets/search.png'),
    'wallet': require('../assets/finance.png'),
    'profile': require('../assets/profile.png')
}

function Navbar({menu}: {menu: string}) {
    function NavbarButton({name}: { name: string }) {
        return (
            <Pressable style={navbarStyles.icon} onPress={() => { console.log(`${name} tab`); }}>
                <Image source={iconPaths[name]} style={{ height:'55%', width:'55%', opacity:(menu == name) ? 1.0 : 0.5 }}/>
            </Pressable>
        );
    }

    return (
        <View style={{
            backgroundColor:colorScheme.secondary,
            height: 40,

            display:'flex',
            justifyContent:'space-evenly',
            alignItems:'flex-end',
            flexDirection:'row'
        }}>
            <NavbarButton name='home' />
            <NavbarButton name='search' />
            <NavbarButton name='wallet' />
            <NavbarButton name='profile' />
        </View>
    );
}

export default Navbar;