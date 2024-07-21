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

import Login from './screens/Login';
import Home from './screens/Home';

const Stack = createNativeStackNavigator();

export const screenSize = {
    height: Dimensions.get("window").height,
    width: Dimensions.get("window").width
}

function App(): React.JSX.Element {
    const isDarkMode = useColorScheme() === 'dark';

    return (
        <NavigationContainer>
            <Stack.Navigator screenOptions={{ headerShown: false }}>
                <Stack.Screen name="login" component={Login} />
                <Stack.Screen name="home" component={Home} />
            </Stack.Navigator>
        </NavigationContainer>
    );
}

export default App;
