# PJToDo 待办事项小应用
#### 背景: 
PJToDo是本人在学习Rust语言的时候突发奇想,既然Rust安全,速度够快,并且跨平台,那么一些需要性能的地方以及核心的公共组件就可以采用Rust来实现.以往大多是采用C/C++来实现这样的需求,这次就试试Rust.PJToDo是本人试验性的小项目.

#### 技术点:
1. 所有数据都存储到github中, 即每次会把ToDo.db上传到github,当从别端登录时拉取ToDo.db,达到伪同步数据效果(主要是没有个人的服务器,通过workaround方式实现数据同步🙈).
2. 网络请求使用Rust, Model使用Rust, 数据库操作使用Rust.
3. 采用MVVM架构.
4. UI层采用iOS实现,理论Android等其他端都可以复用Rust层写的逻辑组件.
5. 更多详细介绍见[Rust in iOS.key](https://github.com/piaojin/PJToDo/blob/master/Rust%20in%20iOS.key)

<img width="1164" alt="image" src="https://github.com/piaojin/PJToDo/assets/10163672/70742a18-cab4-4ee2-9dab-3f3ec5184934">
<img width="1183" alt="image" src="https://github.com/piaojin/PJToDo/assets/10163672/a0bc7ebc-c507-43a0-8853-17f0e711dcb5">
<img width="1176" alt="image" src="https://github.com/piaojin/PJToDo/assets/10163672/7f4eeb15-e31c-48b6-92c5-1f2b5bf82a4f">
<img width="361" alt="image" src="https://github.com/piaojin/PJToDo/assets/10163672/fe9717c2-7c08-40fb-8c9b-f5e30d5216fe">

#### Requirements
* rustc 1.73.0-nightly (1d56e3a6d 2023-07-22)
* cargo 1.73.0-nightly (1b1555676 2023-07-18)
* Swift5.x
* Xcode14.2



