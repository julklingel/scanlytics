import { writable } from 'svelte/store';
import { invoke } from "@tauri-apps/api/core";

interface Auth {
    username: string;
    isValidated: boolean;
}

const initialAuth: Auth = {
    username: '',
    isValidated: false
};

const { subscribe, update } = writable<Auth>(initialAuth);

const AuthService = {
    subscribe,
    login: (username: string) => {
        update(store => ({ ...store, username, isValidated: true }));
    },
    logout: () => {
        update(store => ({ ...store, username: '', isValidated: false }));
    },
    validate: async (): Promise<void> => {
        return new Promise((resolve, reject) => {
            update(store => {
                if (!store.username) {
                    reject(new Error("No username set"));
                    return store;
                }

                invoke("validate_token", { username: store.username })
                    .then(() => {
                        update(s => ({ ...s, isValidated: true }));
                        resolve();
                    })
                    .catch((error) => {
                        update(s => ({ ...s, isValidated: false }));
                        reject(error);
                    });

                return store;
            });
        });
    }
};

export default AuthService;
