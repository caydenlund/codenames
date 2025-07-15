<script>
    const buttonClasses =
        "p-4 flex-1 text-white text-lg font-semibold text-center transition duration-150";
</script>

<h1 class="text-center text-6xl font-bold">Welcome to Codenames!</h1>

<div class="fixed inset-x-0 bottom-0 bg-white pb-[env(safe-area-inset-bottom)] shadow-lg">
    <div class="m-6 flex overflow-hidden rounded-xl">
        <a href="/board" class="{buttonClasses} bg-blue-600 hover:bg-blue-700"> View the board </a>
        <a href="/spymaster" class="{buttonClasses} bg-orange-600 hover:bg-orange-700">
            I'm a spymaster
        </a>
    </div>
</div>
