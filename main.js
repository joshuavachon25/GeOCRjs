const { app, BrowserWindow } = require('electron')
const path = require('node:path')

const isMac = process.platform === 'darwin'
const isDev = process.env.NODE_ENV !== 'production'

const createWindow = () => {
    // Création de la browser window.
    const mainWindow = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            preload: path.join(__dirname, 'preload.js')
        }
    })

    mainWindow.loadFile('renderer/index.html')
}


app.whenReady().then(() => {
    createWindow()

    app.on('activate', () => {
        if (BrowserWindow.getAllWindows().length === 0) createWindow()
    })
})

app.on('window-all-closed', () => {
    if (!isMac) app.quit()
})