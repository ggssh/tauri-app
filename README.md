~~~json
{
  "$schema": "..\\node_modules/@tauri-apps/cli\\schema.json",
  "build": {
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev",
    "devPath": "http://localhost:9999",
    "distDir": "../dist"
  },
  "package": {
    "productName": "tauri-app",
    "version": "0.1.0"
  },
  "tauri": {
    "allowlist": {
      "all": true,
      "fs": {
        "scope": [
          "**"
        ]
      }
    },
    "bundle": {
      "active": true,
      "category": "DeveloperTool",
      "copyright": "",
      "deb": {
        "depends": []
      },
      "externalBin": [],
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "identifier": "com.ggssh.tauri-app",
      "longDescription": "",
      "macOS": {
        "entitlements": null,
        "exceptionDomain": "",
        "frameworks": [],
        "providerShortName": null,
        "signingIdentity": null
      },
      "resources": [],
      "shortDescription": "",
      "targets": "all",
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": "",
        "webviewFixedRuntimePath": "./Microsoft.WebView2.FixedVersionRuntime.102.0.1245.41.x64/"
      }
    },
    "security": {
      "csp": null
    },
    "updater": {
      "active": false
    },
    "windows": [
      {
        "fullscreen": false,
        "height": 600,
        "resizable": true,
        "title": "Tauri App",
        "width": 800
      }
    ]
  }
}
~~~
