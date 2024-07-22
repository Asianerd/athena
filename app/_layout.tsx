import '../gesture-handler';
import { NavigationContainer, StackActions } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import React from 'react';
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
} from 'react-native';

import Login from './login';
import Home from './home';
import { createDrawerNavigator } from '@react-navigation/drawer';

const Stack = createNativeStackNavigator();
const Drawer = createDrawerNavigator();

function StackNavigator() {
    return (
        <Stack.Navigator screenOptions={{ headerShown: false }}>
            <Stack.Screen name="login" component={Login} />
            <Stack.Screen name="home" component={Home} />
        </Stack.Navigator>
    );
}

export const screenSize = {
    height: Dimensions.get("window").height,
    width: Dimensions.get("window").width
}

function App(): React.JSX.Element {
    const isDarkMode = useColorScheme() === 'dark';

    {/* <Drawer.Navigator>
        <Drawer.Screen name="stack" component={StackNavigator} />
    </Drawer.Navigator> */}

    return (
        <Stack.Navigator screenOptions={{ headerShown: false }}>
            <Stack.Screen name="login" component={Login} />
            <Stack.Screen name="home" component={Home} />
        </Stack.Navigator>
    );
}

export default App;