# dailycheck

## 简介

这是一个自动填报西电晨午晚检的程序，仅支持使用 GitHub Actions 部署。

支持南北校区。

填报脚本将会在每天的 8:10, 13:10 和 19:10 自动执行，因不明原因可能会出现误差。

在 Fork 后请及时与上游同步更改。

## 用法

### Fork 本仓库

点击右上角的「Fork」按钮。

### 设置 Secrets

1. 依次点开「Settings → Secrets → Actions」，点击「New repository secret」。
2. Name 填入 `CHECKUP_USERNAME`，Secret 填入你的学号。
3. 点击「Add secret」，此时你已经成功新建了一个 repository secret，Name 为 `CHECKUP_USERNAME`，Secret 为你的学号。
4. 同理，新建一个 repository secret，Name 为 `CHECKUP_PASSWORD`，Secret 为你的密码。
5. 如果你所在的校区是北校区，你需要额外创建一个名为  `CHECKUP_CAMPUS` 的 Secret，其值为 1.

经过以上步骤，脚本就配置完成了。

### 测试 Actions

#### 启用 Actions

1. 点击「Actions」，点击那个大大的绿色按钮。
2. 在左侧选择「Run」，点击「Enable workflow」。
3. 点击「Run workflow」。
4. 等待运行结果即可。如果运行成功会显示绿色的对勾。

如果一切正常，你现在已经可以正常使用了。

#### 查看可能的报错信息

1. 点击你想看的运行记录「Run」。
2. 点击「check up」。
3. 点击显示了红色叉号的步骤。

## 约定

如果身体有任何疑似新冠肺炎症状的情况，请立即停止使用该脚本，并根据实际情况手动填报。

若发生因使用本脚本而导致的任何意外，作者概不负责。