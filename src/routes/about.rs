use leptos::*;
use leptos_meta::*;

use crate::components::footer::{GoBack, HomeFooter};

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="Azrael About"/>
        <Meta name="description" content="About me."/>
        <Body class="bg-[#000000]"/>
        <div class="max-w-3xl px-4 pt-6 lg:pt-10 pb-12 sm:px-6 lg:px-8 mx-auto">
            <div class="max-w-3xl">
                <div class="prose prose-blog mx-auto md:prose-lg prose-pre:m-0 prose-pre:rounded-none">
                    <h2>"我是Azrael.欢迎来到我的博客."</h2>
                    <p>
                        "这是我捣鼓的第二个博客, 之前的博客是用Hexo部署的一个Github Pages页面, 后来由于我懒入职坐班后,逐渐就荒废了."
                    </p>
                    <p>
                        "而后的几个月内有幸接触了"<strong>"Rust"</strong>"这门编程语言, 在结束了基础语法的学习后, 打算通过一些项目继续精进Rust技术."
                        "此时正好看到了Itehax的博客, 逐步了解了leptos框架, 于是通过这个框架重新搭建博客的想法逐步成型, 在一系列踩坑后, 终于发布了该个人网站."
                    </p>
                    <p>
                        "社交媒体是写给别人看的, 博客是写给自己品味的. 在这里你可以看到:"
                    </p>
                    <ol>
                        <li>
                            <p>
                                "计算机科学相关的文章 💻."
                            </p>
                        </li>
                        <li>
                            <p>
                                "游戏评测和攻略 🎮."
                            </p>
                        </li>
                        <li>
                            <p>
                                "个人阅读的观后感,以及一些个人感悟 📚."
                            </p>
                        </li>
                    </ol>
                    <h3>"What should be expected from this blog"</h3>
                    <ol>
                        <li>
                            <p>
                                "Programming post, I have several projects in mind and I will comment/write down what I have learnt useful."
                            </p>
                        </li>
                        <li>
                            <p>
                                "Posts on reverse engineering, malware analysis and game hacking, regarding these topics,I will always post several projects and open a youtube channel, especially on the use of rust in this field 🦀."
                            </p>
                        </li>
                        <li>
                            <p>"Finally, reviews of books read by me."</p>
                        </li>
                    </ol>
                </div>
            </div>
        </div>
        <GoBack content="Back to Home".to_string()/>
        <HomeFooter/>
    }
}
