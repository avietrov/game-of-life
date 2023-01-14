package org.kotlinlang.play         // 1

class Generation(var size: Int) {
    private val grid = MutableList(size * size) { false };

    public fun get(x: Int, y:Int): Boolean? {
        if (x < 0 || x >= size || y < 0 || y >= size)
            return null

        val index = x * size + y;
        return grid[index];
    }

    public fun set(x:Int, y:Int, v: Boolean) {
        val index = x * size + y;
        grid[index] = v;

    }

    public fun neighbours(x: Int, y: Int): List<Boolean?> {
        val u = y - 1;
        val d = y + 1;
        val l = x - 1;
        val r = x + 1;
        return listOf(
            get(l, u), get(x, u), get(r, u),
            get(l, y),            get(r, y),
            get(l, d), get(x, d), get(r, d)
        )
    }

    public fun alive_neighbours(x: Int, y:Int): Int {
       return neighbours(x, y)
            .filter { it != null && it }
            .count()
    }

    public fun evolve(): Generation {
        val next = Generation(size);
        for (x in 0 until size) {
            for (y in 0 until size) {
                val v = when(alive_neighbours(x, y)) {
                    3 -> true
                    2 -> get(x, y) ?: false
                    else -> false
                }
                next.set(x, y, v)
            }
        }
        return next;
    }

    public fun visualise(): String {
        var str = ""
        for (x in 0 until size) {
            for (y in 0 until size) {
                val v = get(x, y)
                val s = if (v != null && v) "x" else " ";
                str += s;
            }
            str += "\n";
        }
        return str;
    }
}

fun main() {
    var g = Generation(10);
    g.set(2, 3, true)
    g.set(3, 3, true)
    g.set(4, 3, true)

    for(i in 0..5) {
        println(g.visualise())
        g = g.evolve();
    }
    println(g.visualise())
}