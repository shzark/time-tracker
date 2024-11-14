
## Build your own time tracker using RUST

- Desktop Application with Tauri 2.x
- App running time
- App resource
- UI with React

- 桌面应用与 Tauri 2.x
- 应用程序运行时间
- App资源
- 使用React的UI


### 步骤

``` 创建APP
 官网:  https://v2.tauri.app/zh-cn/start/create-project/
    1. yarn create tauri-app  
    -  Project name: time-tracker
    -  Identifier: com.time-tracker.app
    - 选择您的前端语言 
    - 选择您的包管理器
    - 选择您的UI模板
    - 选择您的UI风格
```

``` 添加cargo依赖包
    - cd time-tracker/src-tauri
    - cargo add sysinfo
```

```安装 Tauri 的 CLI 工具
    cd ../
    yarn add -D @tauri-apps/cli@latest
```

```编写修改代码文件
    
    rust 文件
        time-tracker/src-tauri/src/lib.rs
        
    tsx文件
        time-tracker/src/App.tsx
        time-tracker/src/ProcessCard.tsx
        time-tracker/src/App.css
    
```

``` 运行开发服务器
    yarn tauri dev
```
