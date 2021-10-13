// Headers
pub const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36";
pub const REFERER: &str = "https://xmuxg.xmu.edu.cn/xmu/login?app=214";

// 每日表单信息接口
pub const CURRENT_FORM_URL: &str = "https://xmuxg.xmu.edu.cn/api/app/214/business/now?getFirst=true";

// 表单字段信息接口
pub const FORM_RENDER_DATA_URL: &str =
  "https://xmuxg.xmu.edu.cn/api/formEngine/business/$1/formRenderData?playerId=owner";

// 用户填写信息接口
pub const MY_FORM_INSTANCE_URL: &str =
  "https://xmuxg.xmu.edu.cn/api/formEngine/business/$1/myFormInstance";

// 提交表单接口
pub const POST_CHANGE_URL: &str = "https://xmuxg.xmu.edu.cn/api/formEngine/formInstance/$1";

// 表单修改记录接口
pub const CHANGELOG_URL: &str = "https://xmuxg.xmu.edu.cn/api/formEngine/formInstances/$1/changeLogs?playerId=owner&businessId=$2";

// 健康码接口 (查询连续打卡天数)
pub const QRCODE_URL: &str = "https://xmuxg.xmu.edu.cn/schoolcustom/qrCode";

// 个人资料接口
pub const PROFILE_URL: &str = "https://xmuxg.xmu.edu.cn/login/check";