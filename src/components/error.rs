use crate::components::footer::HomeFooter;
use http::StatusCode;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn Error(error_code: StatusCode, error_string: String) -> impl IntoView {
    view! {
        <Title text="Error"/>
        <Meta name="description" content="An error occurred."/>
        <div class="max-w-[50rem] flex flex-col mx-auto w-full h-full">
            <header class="mb-auto flex justify-center z-50 w-full py-4">
                <nav class="px-4 sm:px-6 lg:px-8" aria-label="Global">
                    <a
                        class="flex-none text-xl font-semibold sm:text-3xl text-[#F8F9FA]"
                        href="#"
                        aria-label="Brand"
                    >
                        "Itehax"
                    </a>
                </nav>
            </header>

            <div class="text-center py-10 px-4 sm:px-6 lg:px-8">
                <h1 class="block text-3xl font-bold sm:text-3xl text-[#F8F9FA]">
                    {error_code.to_string()}
                </h1>
                <h1 class="block text-2xl font-bold text-[#CED4DA]"></h1>
                <p class="text-gray-400">{error_string}</p>
                <div class="mt-5 flex flex-col justify-center items-center gap-2 sm:flex-row sm:gap-3">
                    <a
                        class="w-full sm:w-auto inline-flex justify-center items-center gap-x-3 text-center bg-blue-600 hover:bg-blue-700 border border-transparent text-white text-sm font-medium rounded-md focus:outline-none focus:ring-2 focus:ring-blue-600 focus:ring-offset-2  transition py-3 px-4 focus:ring-offset-gray-800"
                        href="https://github.com/itehax/itehax-website"
                        target="_blank"
                    >
                        <svg
                            class="w-4 h-4"
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            fill="currentColor"
                            viewBox="0 0 16 16"
                        >
                            <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"></path>

                        </svg>
                        "Get the source code"
                    </a>
                    <a
                        class="w-full sm:w-auto inline-flex justify-center items-center gap-2 rounded-md border border-transparent font-semibold text-blue-500 hover:text-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm py-3 px-4 ring-offset-slate-900"
                        href="/"
                    >
                        <svg
                            class="w-2.5 h-2.5"
                            width="16"
                            height="16"
                            viewBox="0 0 16 16"
                            fill="none"
                        >
                            <path
                                d="M11.2792 1.64001L5.63273 7.28646C5.43747 7.48172 5.43747 7.79831 5.63273 7.99357L11.2792 13.64"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                            ></path>
                        </svg>
                        "Back to Home"
                    </a>
                </div>
            </div>
            <HomeFooter father_name="Error".to_string()/>
        </div>
    }
}
