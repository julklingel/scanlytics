import { writable } from 'svelte/store';
import { invoke } from "@tauri-apps/api/core";

interface Auth {
    user_email: string;
    isValidated: boolean;
}

const initialAuth: Auth = {
    user_email: '',
    isValidated: false
};

const { subscribe, update } = writable<Auth>(initialAuth);

const AuthService = {
    subscribe,
    login: (user_email: string) => {
        update(store => ({ ...store, user_email, isValidated: true }));
    },
    logout: () => {
        update(store => ({ ...store, user_email: '', isValidated: false }));
    },
    validate: async (): Promise<void> => {
        return new Promise((resolve, reject) => {
            update(store => {
                if (!store.user_email) {
                    reject(new Error("No user_email set"));
                    return store;
                }

                invoke("validate_token", { user_email: store.user_email })
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
