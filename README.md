# dailycheck

## 简介

这是一个自动填报西电晨午晚检的程序，仅支持使用 GitHub Actions 部署。

仅支持南校区。

## 用法

### Fork 本仓库

点击右上角的「Fork」按钮。

### 设置 Secrets

1. 依次点开「Settings → Secrets → Actions」，点击「New repository secret」。
2. Name 填入 `CHECKUP_USERNAME`，Secret 填入你的学号。
3. 点击「Add secret」，此时你已经成功新建了一个 repository secret，Name 为 `CHECKUP_USERNAME`，Secret 为你的学号。
4. 同理，新建一个 repository secret，Name 为 `CHECKUP_PASSWORD`，Secret 为你的密码。

经过以上步骤，脚本就配置完成了。

### 测试 Actions

#### 启用 Actions

1. 点击「Actions」，点击那个大大的绿色按钮。
2. 在左侧选择「Run」，点击「Enable workflow」。
3. 点击「Run workflow」。
4. 等待运行结果即可。如果运行成功会显示绿色的对勾。

如果一切正常，你现在已经可以正常使用了。