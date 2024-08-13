import nonebot
from nonebot.adapters.onebot.v11 import Adapter as onebotAdapter  # 避免重复命名

nonebot.init()

driver = nonebot.get_driver()
driver.register_adapter(onebotAdapter)

nonebot.load_plugin("nonebot_plugin_cube")

if __name__ == "__main__":
    nonebot.run()
