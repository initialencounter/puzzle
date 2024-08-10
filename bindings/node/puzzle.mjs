const drctn_dist = { 'U': [-1, 0], 'D': [1, 0], 'L': [0, -1], 'R': [0, 1] }
const drctn_list = ['U', 'D', 'L', 'R']
export class Klotsk {
    constructor(mode) {
        this.mode = mode
        this.klotsk = []
        this.num = 1
        this.crtpuzzle = []
        for (let i = 0; i < this.mode; i++) {  // 生成数组
            var temp = []
            for (let j = 0; j < this.mode; j++) {
                temp.push(this.num)
                this.num++
            }
            this.crtpuzzle.push([].concat(temp))
            this.klotsk.push([].concat(temp))
        }
        this.crtpuzzle[this.mode - 1][this.mode - 1] = 0;
        this.klotsk[this.mode - 1][this.mode - 1] = 0;
        this.start_time = new Date().getTime()               //计时开始
        this.end_time = 0
        this.shfl()
    }

    find_0() {
        for (let i = 0; i < this.mode; i++) {                      // 找出空的位置
            for (let j = 0; j < this.mode; j++) {
                if (this.klotsk[i][j] == 0) {
                    return [i, j]
                }
            }
        }
    }
    move(derect) {                        // 移动
        const [r, c] = this.find_0()
        if (r == 0 && derect == 'U') {  // 判断是否可以移动
            return ''
        }
        else if (r == this.mode - 1 && derect == 'D') {
            return ''
        }
        else if (c == 0 && derect == 'L') {
            return ''
        }
        else if (c == this.mode - 1 && derect == 'R') {
            return ''
        }
        const [rr, cc] = drctn_dist[derect]

        const num1= this.klotsk[rr + r][cc + c]
        this.klotsk[r][c] = num1
        this.klotsk[rr + r][cc + c] = 0
        return derect
    }
    check() {
        for (let i = 0; i < this.mode; i++) {                          // 生成数组
            for (let j = 0; j < this.mode; j++) {
                if (this.klotsk[i][j] != this.crtpuzzle[i][j]) {
                    return false
                }
            }
        }
        return true
    }
    move_sqnc(sqnc) {   //移动命令序列，并判断是否还原
        this.cmd_strs = ''
        const sqnc_arr = sqnc.split('')
        for (var i in sqnc_arr) {
            var com_str = this.move(sqnc_arr[i])
            this.cmd_strs += com_str
            if (this.check()) {
                const dt = this.duration()
                return true
            }
        }
        return false
    }
    shfl() {
        for (let i = 0; i < 1000; i++) {     // 打乱puzzle
            const rd = (Math.random() * 3).toFixed(0)
            this.move(drctn_list[rd])
        }
    }
    duration() {                  //计时方法
        const time = new Date().getTime()
        const dt = time - this.start_time
        return this.strftime(dt)
    }
    strftime(num_time) {   //时间转换
        const h = Math.floor(num_time / 3600)
        const m= Math.floor((num_time % 3600) / 60)
        const s = Math.floor((num_time % 60))
        const strftime = `${h}:${m}:${s}`
        return strftime
    }
}