import '../gesture-handler';
import { useFonts } from 'expo-font';
import * as SplashScreen from 'expo-splash-screen';
import { NavigationContainer, StackActions, getFocusedRouteNameFromRoute } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import React, { useEffect, useState } from 'react';
import type {PropsWithChildren} from 'react';
import {
    SafeAreaView,
    ScrollView,
    Text,
    useColorScheme,
    View,
    Button,
    Dimensions,
    Image,
    Pressable,
} from 'react-native';

import Login from './login';
import Home from './home';
import { createDrawerNavigator } from '@react-navigation/drawer';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import { colorScheme, defaultFont, defaultFontBold, fontSize } from '@/constants/style';
import { CustomDrawerContent } from '@/components/Sidebar';

import GLOBALS from './global';
import { Project } from '@/constants/Project';

const Stack = createNativeStackNavigator();
const Drawer = createDrawerNavigator();

export const screenSize = {
    height: Dimensions.get("window").height,
    width: Dimensions.get("window").width
}

export async function fetchProjects(changeListFunction: any) {
    try {
        let response = await Project.fetchProjects();
        changeListFunction(response);
    } catch (e) {
        console.log(`fetchProjects() -> error found : ${e}`);
    } finally {
        
    }
}

function App(): React.JSX.Element {
    const isDarkMode = useColorScheme() === 'dark';
    const safeAreaInsets = useSafeAreaInsets();

    const [fontLoaded, fontError] = useFonts({
        'SplineSansMono-Light': require('../assets/fonts/SplineSansMono-Light.ttf'),
        'SplineSansMono-LightItalic': require('../assets/fonts/SplineSansMono-LightItalic.ttf'),
        'SplineSansMono-SemiBold': require('../assets/fonts/SplineSansMono-SemiBold.ttf')
    });

    useEffect(() => {
        if (fontLoaded || fontError) {
            SplashScreen.hideAsync();
        }

        fetchProjects(changeProjectList);
    }, [fontLoaded, fontError]);

    const [selection, localChangeSelection] = useState<Project | undefined>(GLOBALS.selectedProject);
    const changeSelection = (t: undefined) => {
        localChangeSelection(t);
        GLOBALS.selectedProject = t;
    }

    const [projectList, localChangeProjectList] = useState<{[id: number]: Project}>([]);
    const changeProjectList = (l: any) => {
        localChangeProjectList(l);
        GLOBALS.projectList = l;
    }

    function StackNavigator() {
        return (
            <Stack.Navigator screenOptions={{ headerShown: false, gestureEnabled: false }}>
                <Stack.Screen name="login" component={Login} />
                <Stack.Screen name="home" component={Home} />
            </Stack.Navigator>
        );
    }

    return (
        <Drawer.Navigator drawerContent={() => { return CustomDrawerContent(safeAreaInsets, selection, changeSelection, projectList, changeProjectList); }} screenOptions={{
            headerShown:false
        }}>
            <Drawer.Screen name="stack" component={StackNavigator} options={({route}: {route:any}) => {
                const routeName = getFocusedRouteNameFromRoute(route) ?? 'login';
                return ({swipeEnabled: (routeName != 'login')});
            }}/>
        </Drawer.Navigator>
    );
}

export default App;