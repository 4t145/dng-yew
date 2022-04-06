use super::Locals;


const HELP:&str = 
r#"help> 帮助
switch to English: /lang en

# 快捷键
crtl+z 撤销，crtl+z 重做，crtl+x 清屏
在铅笔模式下， 右键按下使用橡皮擦
E键可以在铅笔和橡皮擦之间切换, 主要是方便数位笔使用者
右键调色板小方块可以修改调色板颜色

# 关键命令
/ready 准备
/unready 取消准备
/name <名字> 设置名字
/lexicon <文件网址> 从github上下载词库
/lexicon <词库代码> 从词库服务器设置词库

# 可用中文词库
https://github.com/4t145/dng-lex/tree/main/zh

更详细帮助参考：https://github.com/4t145/dng-yew
/help 调出此帮助
"#;

pub const ZH:Locals = Locals {
    me: "我",

    game_start: "游戏开始",
    game_end: "游戏结束",
    turn_start: "回合开始",
    turn_end: "回合结束",
    mark_start: "评分开始",
    mark_end: "评分结束",

    mark: "打个分吧",
    vote_down: "又摆摆 --p",
    vote_neutral: "一般般 o_o",
    vote_up: "优棒棒 ^^b",

    key_word: "关键词：",

    unsupported: "不支持的命令",
    check_your_input: "请检查你的输入是否正确",
    lack_of_parameter: "缺少参数",

    input_placeholder: "在此输入",

    help: HELP
};