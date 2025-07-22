<div class="flex min-h-screen items-center justify-center p-4">
    <div class="w-full max-w-4xl">
        <div class="mb-12 text-center">
            <h1 class="mb-4 text-6xl font-bold text-gray-800">Codenames</h1>
            <p class="text-xl text-gray-500">Choose your role and join the game</p>
        </div>

        <div class="mb-8 grid gap-6 md:grid-cols-2">
            <div
                class="transform rounded-2xl bg-white p-8 shadow-md transition-all duration-300 hover:-translate-y-1 hover:shadow-xl"
            >
                <div class="mb-6 text-center">
                    <div
                        class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-blue-100"
                    >
                        <svg
                            class="h-8 w-8 text-blue-600"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
                            ></path>
                        </svg>
                    </div>
                    <h2 class="mb-2 text-2xl font-bold text-gray-800">Player View</h2>
                    <p class="mb-6 text-gray-600">
                        Join as a field operative and see the public board. Work with your team to
                        identify your agents using only one-word clues.
                    </p>
                </div>

                <a
                    href="/board"
                    class="block w-full transform rounded-xl bg-blue-600 px-6 py-4 text-center font-semibold text-white transition-all duration-200 hover:scale-105 hover:bg-blue-700 active:scale-95"
                >
                    Enter Game Board
                </a>
            </div>

            <div
                class="transform rounded-2xl bg-white p-8 shadow-md transition-all duration-300 hover:-translate-y-1 hover:shadow-xl"
            >
                <div class="mb-6 text-center">
                    <div
                        class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-red-100"
                    >
                        <svg
                            class="h-8 w-8 text-red-600"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                            ></path>
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                            ></path>
                        </svg>
                    </div>
                    <h2 class="mb-2 text-2xl font-bold text-gray-800">Spymaster</h2>
                    <p class="mb-6 text-gray-600">
                        Take control as the spymaster. See all agent locations and guide your team
                        to victory while avoiding the assassin.
                    </p>
                </div>

                <a
                    href="/spymaster"
                    class="block w-full transform rounded-xl bg-red-600 px-6 py-4 text-center font-semibold text-white transition-all duration-200 hover:scale-105 hover:bg-red-700 active:scale-95"
                >
                    Enter Spymaster View
                </a>
            </div>
        </div>
    </div>
</div>
