import { Button, Image, Pressable, ScrollView, StyleSheet, Text, TextInput, View } from "react-native";
import { screenSize } from "../App";
import React from "react";
import { colorScheme, defaultFont, defaultFontBold, defaultFontItalic, fontSize } from "../constants/style";
import { SOTERIUS_BACKEND } from "../constants/networking";

function Login({navigation}: {navigation: any}): React.JSX.Element {
    navigation.navigate('home', { username:'han_yuji_', password: 'chronos' });
    // here for debugging only

    function attemptLogin(username: String, password: String) {
        fetch(`${SOTERIUS_BACKEND}/${state}`, {
            method: 'POST',
            body: JSON.stringify({
                username: username,
                password: password
            }),
            headers: {
                'Content-type': 'text/plain'
            }
        })
        .then(response => response.json())
        .then(
            json => {
                if (JSON.parse(JSON.stringify(json))['Success'] != undefined) {
                    // successful login
                    console.log("logged in");
                    navigation.navigate('home', { username:username, password: password });
                    onStatusChange('');
                } else {
                    let message = {
                        'PasswordWrong': 'password is wrong',
                        'UsernameNoExist': "username doesn't exist",
                        'UsernameTaken': 'that username is already taken',
                        'PasswordNoExist': 'i dont know either'
                    }[JSON.stringify(json).replaceAll('"', '')];
                    onStatusChange(message != undefined ? message : '');
                }
            }
        )
        .catch((error) => {
            console.log(`attemptLogin() at Login.tsx error : ${error}`);
        })
    }

    const loginStyles = StyleSheet.create({
        input: {
            width:0.7 * screenSize.width,

            borderStyle: "solid",
            borderColor: colorScheme.border,
            borderBottomWidth: 1,

            paddingTop:20,
            paddingBottom:5,

            color: colorScheme.primary,

            fontSize: fontSize.small,
            fontFamily: defaultFont
        },
        button: {
            backgroundColor: colorScheme.secondary,
            padding:8,
            paddingHorizontal:15,
            borderRadius:10,

            overflow:'hidden'
        }
    });

    const [state, onStateChange] = React.useState('login');

    const [username, onUsernameChange] = React.useState('');
    const [password, onPasswordChange] = React.useState('');
    const [confirmPassword, onConfirmPasswordChange] = React.useState('');

    const [status, onStatusChange] = React.useState('');

    const validatePassword = () => {
        if (password != confirmPassword) {
            return {
                state: false,
                message: 'passwords do not match'
            };
        }

        return {
            state: true,
            message: ''
        };
    }

    return <View style={{
        backgroundColor: colorScheme.background,
        flex: 1,
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center'
        }}>
            {/* <Image source={require('../assets/sns/github.png')} style={{
                borderRadius: 1000,
                height:'20%',
                aspectRatio: 1,
                marginBottom:20
            }} /> */}
            <Text style={{ color:colorScheme.tertiary, fontSize: fontSize.large, fontFamily:defaultFontBold, marginBottom:20 }}>
                athena
            </Text>
            <TextInput style={loginStyles.input}
                placeholder="username"
                onChangeText={onUsernameChange}
                spellCheck={false}
            />
            <TextInput style={loginStyles.input}
                placeholder="password"
                onChangeText={onPasswordChange}
                spellCheck={false}
                secureTextEntry={true}
            />
            <TextInput style={[ loginStyles.input,
                    {
                        display:(state == 'login' ? 'none' : 'flex')
                    }
                ]}
                placeholder="confirm password"
                onChangeText={onConfirmPasswordChange}
                spellCheck={false}
                secureTextEntry={true}
            />
            <Text style={{ fontFamily:defaultFontItalic, color:colorScheme.error, marginTop: 20 }}>
                {status}
            </Text>
            <View style={{
                width: 0.7 * screenSize.width,
                display: 'flex',
                alignItems: 'center',
                flexDirection: 'column',

                marginTop:35,
            }}>
                <Pressable style={loginStyles.button} onPress={() => {
                    console.log(`login by ${username} with ${password}`);
                    if (state == 'signup') {
                        let result = validatePassword();
                        if (!result.state) {
                            onStatusChange(result.message);
                            return;
                        }
                    }

                    attemptLogin(username, password);
                }}>
                    <Text style={{ color:colorScheme.primary, fontSize: fontSize.medium, fontFamily:defaultFont }}>
                        {
                            state == 'login' ? 'log in' : 'sign up'
                        }
                    </Text>
                </Pressable>
                <Pressable style={{ marginTop:10 }} onPress={() => { onStateChange(state == 'login' ? 'signup' : 'login') }}>
                    <Text style={{ color:colorScheme.link, fontSize: fontSize.tiny, fontFamily: defaultFontItalic }}>
                        {
                            state == 'login' ? 'dont have an account? sign up' : 'have an account? log in'
                        }
                    </Text>
                </Pressable>
            </View>
    </View>
}

export default Login;
