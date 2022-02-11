const fs = require('fs')

const make_some_noise = async () => {
    let numbers = []
    for (let i = 0; i < 1000000; i++) {
        numbers.push(Math.floor(Math.random() * 1000000))
    }
    await write_the_thing(numbers)
}

const write_the_thing = async (array) => {
    fs.writeFileSync("numbers.txt", `${array}`, (err) => { if (err) throw err })
}

const two_sum_naive = (nums, target) => {
    const start = Math.floor(Date.now())
    for (let i = 0; i < nums.length; i++) {
        const num = nums[i]
        for (let j = i + 1; j < nums.length; j++) {
            if (num + nums[j] === target) {
                const end = Math.floor(Date.now())
                console.log(`JS Time: ${end - start}ms`)
                console.log(`JS Solution: ${nums[i]}, ${nums[j]}`)
                return [i, j]
            }
        }
    }
}


const two_sum = (nums, target) => {

    let sums = []
    let hash_table = {}

    const start = Math.floor(Date.now())
    for (let i = 0; i < nums.length; i++) {
        const num = nums[i]
        let complement = target - num

        // if complement is in hash_table, then we have a solution
        if (hash_table[complement.toString()] !== undefined) {
            sums.push([num, complement])
        }

        // add num to hash_table
        hash_table[num.toString()] = nums[i]
    }
    const end = Math.floor(Date.now())
    sums.map(sum => console.log(`Possible: ${sum[0]}, ${sum[1]}`))
    console.log(`JS Time: ${end - start}ms`)
    return sums
}

async function main() {

    // await make_some_noise()

    const data = fs.readFileSync("numbers.txt", "ascii")

    two_sum(data.split(",").map(Number), 13)
}


main()
