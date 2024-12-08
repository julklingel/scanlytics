import { writable } from 'svelte/store';
import { invoke } from "@tauri-apps/api/core";

interface Auth {
    user_email: string;
    isValidated: boolean;
}

const storedAuth = localStorage.getItem('auth');
const initialAuth: Auth = storedAuth 
    ? JSON.parse(storedAuth) 
    : {
        user_email: '',
        isValidated: false
    };

const { subscribe, update } = writable<Auth>(initialAuth);

const AuthService = {
    subscribe,
    login: (user_email: string) => {
        update(store => {
            const newState = { ...store, user_email, isValidated: true };
            localStorage.setItem('auth', JSON.stringify(newState));
            return newState;
        });
    },
    logout: () => {
        update(store => {
            const newState = { ...store, user_email: '', isValidated: false };
            localStorage.removeItem('auth');
            return newState;
        });
    },
    validate: async (): Promise<void> => {
        return new Promise((resolve, reject) => {
            update(store => {
                if (!store.user_email) {
                    reject(new Error("No user_email set"));
                    return store;
                }

                invoke("validate_token", { userEmail: store.user_email })
                    .then(() => {
                        update(s => {
                            const newState = { ...s, isValidated: true };
                            localStorage.setItem('auth', JSON.stringify(newState));
                            return newState;
                        });
                        resolve();
                    })
                    .catch((error) => {
                        update(s => {
                            const newState = { ...s, isValidated: false };
                            localStorage.setItem('auth', JSON.stringify(newState));
                            return newState;
                        });
                        reject(error);
                    });

                return store;
            });
        });
    }
};

export default AuthService;
