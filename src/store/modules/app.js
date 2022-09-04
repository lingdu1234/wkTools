import { defineStore } from "pinia";
import { invoke } from '@tauri-apps/api/tauri';
import { getName, getTauriVersion, getVersion } from '@tauri-apps/api/app';

const useAppStore = defineStore(
    "app",
    {
        state: () => ({
            updatable: false,
            appInfo: {
                name: "",
                appVersion: "",
                tauriVersion: "",
            },
            opacity: 1000,
            lang: null,
            updateInfo: {
                version: null,
                date: null,
                body: null
            },
            routeInfo: {
                title: "route.home",
                path: "/index"
            }


        }),
        persist: {
            paths: ["updatable", "appInfo", "updateInfo", "lang", "routeInfo", "opacity"]
        },
        actions: {
            async appInit() {
                // appInfo获取
                this.appInfo.name = await getName();
                this.appInfo.tauriVersion = await getTauriVersion();
                this.appInfo.appVersion = await getVersion();
            },
            setUpdateInfo(updateInfo) {
                this.updateInfo = {
                    version: updateInfo.version,
                    date: updateInfo.date,
                    body: updateInfo.body
                }
            },
            setUpdatable(updatable) {
                this.updatable = updatable
            },
            async setLang(v) {
                this.lang = v
                await invoke('set_lang', { lang: v });
            },
            setRouteInfo(routeInfo) {
                this.routeInfo = routeInfo
            },


        }
    }
)


export default useAppStore