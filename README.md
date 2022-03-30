# dng-yew
你画我猜的网页客户端

## 使用
### 加入房间
访问`<webset_url>/?server=<server>`，其中`<server>`是房间服务器的ip+端口，如`127.0.0.1:9000`
![图片](https://user-images.githubusercontent.com/34513116/160798411-866faf6b-59ab-4f11-a306-855f1f451118.png)
### 设置词库
[这](https://github.com/4t145/dng-lex)是我个人的词库的仓库，
在聊天窗口中输入`/lexicon <github-url>`可以设置来自github上的词库，如：
```
/lexicon https://github.com/4t145/dng-lex/blob/main/chengyu.json
```
具体的词库格式请参考[这](https://github.com/4t145/dng-lex/blob/main/chengyu.json)，请确保每个字段都存在哦。

⚠免责声明：您使用的网络词库与本项目没有任何关系，本项目仅仅提供加载，并不知道您的词库里有何具体内容。

在聊天窗口中输入`/lexicon <lexcode>`可以设置来自词库服务器上的词库，如：
```
/lexicon 1a2b3c4f
```

### 使用命令
在聊天窗口输入`/help`来查看目前支持的命令（当然你也可以查看[相关源代码](./src/components/console/mod.rs)）
#### 为什么没有按钮
当然是因为没有做！如果你愿意帮忙画一个按钮，并且指出它应当安放在某个合理的位置，那么非常欢迎！


# 自己部署
## 服务器
这个仓库是前端项目，相关的服务器程序和具体的使用说明在[这里](https://github.com/4t145/dng-server)


# 编译
如果你想自己编译这个网站，并且已经安装了rust和wasm工具链，那么应当只需要：
```bash
cargo install trunk
trunk build
```
如果你没有这些，那么你可以在rust官网找到相关安装操作。

