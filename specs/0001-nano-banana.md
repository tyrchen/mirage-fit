# Nano Banana (Gemini 2.5 Flash Image Preview) 完整调研报告

## 概述

**Nano Banana** 是 Google 的突破性 AI 图像编辑模型，官方名称为 **Gemini 2.5 Flash Image Preview**。该模型能够以出色的精度生成和编辑图像，在多次修改中保持主体的相似性。"nano-banana" 是 Google 内部使用的代号，现已成为他们最先进的图像生成和编辑模型。

## 核心功能

### 主要特性
- **多图像融合**：将多个图像混合成单一图像
- **角色一致性**：在丰富的叙事中保持角色一致性
- **目标转换**：使用自然语言进行有针对性的转换
- **世界知识集成**：使用 Gemini 的世界知识生成和编辑图像
- **推理能力集成**：模型在应用编辑前会"思考"，避免扭曲特征或不匹配光照等常见问题

## API 调用文档

### 基础配置

**基础 API 端点**：
```
https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-image-preview:generateContent
```

**认证方式**：
- 使用 Google AI Studio 生成 API Key
- 每日免费配额：1,500 次请求（测试用）
- Header 中使用 `x-goog-api-key` 进行认证

### 基础文本生图 API 调用

```bash
curl -s -X POST "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-image-preview:generateContent" \
  -H "x-goog-api-key: $GEMINI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "contents": [{
      "parts": [
        {"text": "在高档餐厅中创建一个以双子座为主题的纳米香蕉菜品图片"}
      ]
    }],
    "generationConfig": {
      "responseModalities": ["TEXT", "IMAGE"]
    }
  }' \
  | grep -o '"data": "[^"]*"' \
  | cut -d'"' -f4 \
  | base64 --decode > generated_image.png
```

### 图像编辑 API 调用

对现有图像进行编辑的 curl 命令：

```bash
# 准备图像文件
IMG_PATH="/path/to/your_image.jpeg"

# 根据系统类型设置 base64 标志
if [[ "$(base64 --version 2>&1)" = *"FreeBSD"* ]]; then
  B64FLAGS="--input"
else
  B64FLAGS="-w0"
fi

# 将图像转换为 base64
IMG_BASE64=$(base64 "$B64FLAGS" "$IMG_PATH" 2>&1)

# API 调用
curl -X POST \
  "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-image-preview:generateContent" \
  -H "x-goog-api-key: $GEMINI_API_KEY" \
  -H 'Content-Type: application/json' \
  -d "{
    \"contents\": [{
      \"parts\":[
        {\"text\": \"让这只猫在双子座星空下的高档餐厅里吃纳米香蕉\"},
        {
          \"inline_data\": {
            \"mime_type\":\"image/jpeg\",
            \"data\": \"$IMG_BASE64\"
          }
        }
      ]
    }],
    \"generationConfig\": {
      \"responseModalities\": [\"TEXT\", \"IMAGE\"]
    }
  }" \
  | grep -o '"data": "[^"]*"' \
  | cut -d'"' -f4 \
  | base64 --decode > edited_image.png
```

### 多图像组合 API 调用

```bash
# 准备多个图像文件
IMG1_BASE64=$(base64 -w0 "/path/to/first_image.jpg")
IMG2_BASE64=$(base64 -w0 "/path/to/second_image.jpg")
IMG3_BASE64=$(base64 -w0 "/path/to/third_image.jpg")

curl -X POST \
  "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-image-preview:generateContent" \
  -H "x-goog-api-key: $GEMINI_API_KEY" \
  -H 'Content-Type: application/json' \
  -d "{
    \"contents\": [{
      \"parts\":[
        {\"text\": \"将这三个图像融合成一个连贯的场景，保持每个主体的特征但创造新的构图\"},
        {
          \"inline_data\": {
            \"mime_type\":\"image/jpeg\",
            \"data\": \"$IMG1_BASE64\"
          }
        },
        {
          \"inline_data\": {
            \"mime_type\":\"image/jpeg\",
            \"data\": \"$IMG2_BASE64\"
          }
        },
        {
          \"inline_data\": {
            \"mime_type\":\"image/png\",
            \"data\": \"$IMG3_BASE64\"
          }
        }
      ]
    }]
  }" > multi_image_response.json
```

## 高级应用：Prompt Remix 多图像技术

### Prompt Remix 核心概念

Prompt Remix 是一种高级技术，允许用户通过自然语言指令对多个图像进行创意混合和重构。这种技术特别适用于：

1. **风格转换**：从一个图像提取风格，应用到另一个图像
2. **场景融合**：将不同场景中的元素组合成新的构图
3. **角色一致性维护**：在不同环境中保持同一角色的外观
4. **产品展示**：将产品放置到新的环境中

### 高级 Prompt Remix 策略

#### 1. 叙述性描述法

**核心原则**：描述场景，而不是列举关键词。叙述性的描述段落几乎总能产生比断开的词汇列表更好、更连贯的图像。

```json
{
  "contents": [{
    "parts": [
      {
        "text": "创建一个摄影级真实的中景镜头，展示一位优雅的女性在巴黎咖啡馆中品尝纳米香蕉甜点。场景被温暖的金色下午阳光照亮，营造出浪漫的氛围。使用 85mm 镜头拍摄，强调精致的纹理和细节。图像应为 16:9 宽屏格式。"
      },
      {
        "inline_data": {
          "mime_type": "image/jpeg",
          "data": "base64_encoded_portrait"
        }
      },
      {
        "inline_data": {
          "mime_type": "image/jpeg",
          "data": "base64_encoded_cafe_scene"
        }
      }
    ]
  }]
}
```

#### 2. 摄影风格提示法

对于逼真图像，使用摄影术语。提及相机角度、镜头类型、照明和精细细节：

```bash
curl -X POST \
  "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-image-preview:generateContent" \
  -H "x-goog-api-key: $GEMINI_API_KEY" \
  -H 'Content-Type: application/json' \
  -d "{
    \"contents\": [{
      \"parts\":[
        {\"text\": \"专业摄影工作室拍摄：广角镜头下的[主体]，[动作或表情]，置于[环境]中。场景由[照明描述]照亮，营造[情绪]氛围。使用[相机/镜头细节]拍摄，强调[关键纹理和细节]。图像应为[宽高比]格式。\"},
        {\"inline_data\": {\"mime_type\":\"image/jpeg\", \"data\":\"$IMG_BASE64\"}}
      ]
    }]
  }"
```

#### 3. 角色一致性维护技术

使用短语"这个确切的[角色]"结合特定特征描述：

```json
{
  "text": "保持这个确切角色的面部特征，将其放置在未来主义城市环境中，维持相同的几何形态和独特标识，确保面部细节的完美一致性"
}
```

#### 4. 迭代式对话编辑

支持多轮对话式编辑，逐步完善图像：

```bash
# 第一轮：上传蓝色汽车图像
curl -X POST "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-image-preview:generateContent" \
  -H "x-goog-api-key: $GEMINI_API_KEY" \
  -H 'Content-Type: application/json' \
  -d '{"contents": [{"parts": [{"text": "将这辆车改造成敞篷车"}, {"inline_data": {"mime_type":"image/jpeg", "data":"'$BLUE_CAR_BASE64'"}}]}]}'

# 第二轮：基于第一轮结果继续编辑
curl -X POST "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-image-preview:generateContent" \
  -H "x-goog-api-key: $GEMINI_API_KEY" \
  -H 'Content-Type: application/json' \
  -d '{"contents": [{"parts": [{"text": "现在将颜色改为黄色，并在车身上添加运动条纹"}, {"inline_data": {"mime_type":"image/jpeg", "data":"'$CONVERTIBLE_BASE64'"}}]}]}'
```

### 高级多图像组合模式

#### 模式 1：环境替换
```json
{
  "text": "将第一张图像中的人物提取出来，放置到第二张图像的海滩环境中，确保光照和阴影自然融合，保持人物原有的服装和姿态"
}
```

#### 模式 2：风格迁移
```json
{
  "text": "将第二张图像的艺术风格和色调应用到第一张图像上，创造出具有相同美学特征但保持原始主题的新作品"
}
```

#### 模式 3：产品展示合成
```json
{
  "text": "将第一张图像中的产品无缝集成到第二张图像的豪华展厅环境中，调整大小和角度以符合空间透视，添加适当的反射和光影效果"
}
```

## Python SDK 示例

```python
from google import genai
from PIL import Image
from io import BytesIO
import base64

# 初始化客户端
client = genai.Client()

def multi_image_remix(prompt, image_paths):
    """多图像 Prompt Remix 功能"""
    contents = [prompt]

    # 添加多个图像
    for path in image_paths:
        with open(path, "rb") as image_file:
            image_data = base64.b64encode(image_file.read()).decode('utf-8')
            contents.append({
                "inline_data": {
                    "mime_type": "image/jpeg",
                    "data": image_data
                }
            })

    # 生成内容
    response = client.models.generate_content(
        model="gemini-2.5-flash-image-preview",
        contents=contents,
    )

    # 处理响应
    for part in response.candidates[0].content.parts:
        if part.inline_data is not None:
            image = Image.open(BytesIO(part.inline_data.data))
            return image

    return None

# 使用示例
prompt = """
创建一个梦幻般的场景，将第一张图像中的角色放置在第二张图像的魔法森林中，
同时融入第三张图像的色彩调色板。确保整体构图和谐统一，光线自然过渡。
"""

image_paths = [
    "/path/to/character.jpg",
    "/path/to/forest.jpg",
    "/path/to/color_reference.jpg"
]

result_image = multi_image_remix(prompt, image_paths)
if result_image:
    result_image.save("remix_result.png")
```

## 高级应用场景

### 1. 电商产品展示
- **应用**：将产品图像与不同场景结合，创建多样化的展示效果
- **技术**：环境替换 + 光照匹配
- **示例提示**：`"将这款手表精确地放置在奢华办公桌场景中，确保反射和光影真实自然"`

### 2. 品牌视觉一致性
- **应用**：在不同环境中保持品牌角色或吉祥物的一致外观
- **技术**：角色一致性维护 + 场景适配
- **示例提示**：`"保持这个确切的品牌角色，将其放置在春季花园中，维持相同的颜色方案和特征细节"`

### 3. 创意内容制作
- **应用**：为社交媒体、广告创建独特的视觉内容
- **技术**：风格迁移 + 创意组合
- **示例提示**：`"将现代都市的活力色彩和纹理应用到这个传统建筑上，创造出古今融合的视觉效果"`

### 4. 教育和培训材料
- **应用**：创建定制化的教学图像和示例
- **技术**：概念可视化 + 场景构建
- **示例提示**：`"将这些科学概念图表融合到实际实验室环境中，创建直观的学习材料"`

## 定价和技术规格

### 定价模型
- **价格**：每百万输出 token $30.00
- **每张图像**：1290 个输出 token（约 $0.039/张）
- **免费配额**：每日 1,500 次请求（通过 Google AI Studio）

### 技术规格
- **支持的图像格式**：JPEG, PNG, WEBP, HEIC, HEIF
- **最大图像尺寸**：参考 Image understanding 页面
- **多图像支持**：最多 3 张图像同时处理
- **输出格式**：Base64 编码的图像数据
- **水印**：所有图像包含 SynthID 不可见数字水印

### 安全和合规性
- **内容安全**：内置安全防护措施
- **水印技术**：SynthID 可见和不可见水印标识 AI 生成内容
- **隐私保护**：符合 Google 隐私政策
- **使用限制**：禁止生成有害、误导性或违法内容

## 最佳实践和技巧

### Prompt 工程最佳实践

1. **使用叙述性描述**：用完整句子描述场景，避免关键词堆砌
2. **包含摄影术语**：角度、镜头、光照、情绪等专业术语
3. **具体化细节**：纹理、材质、环境、氛围等具体描述
4. **保持逻辑一致性**：确保描述的元素在物理上和逻辑上合理

### 多图像处理技巧

1. **图像质量匹配**：确保输入图像分辨率和质量相近
2. **风格协调**：选择风格兼容的图像进行混合
3. **光照一致性**：考虑光源方向和强度的协调
4. **色彩平衡**：注意整体色彩方案的和谐统一

### 性能优化

1. **图像压缩**：适当压缩输入图像以减少 token 消耗
2. **批量处理**：对于大量图像，考虑批量处理策略
3. **缓存结果**：保存成功的配置和结果以供重复使用
4. **错误处理**：实现重试机制和优雅的错误处理

## 故障排除和常见问题

### 常见错误码
- **400 Bad Request**：检查 JSON 格式和 Base64 编码
- **401 Unauthorized**：验证 API Key 有效性
- **429 Too Many Requests**：达到速率限制，稍后重试
- **500 Internal Server Error**：服务器临时问题，重试请求

### 质量优化建议
- **提示词过于简单**：增加描述性细节和上下文
- **图像融合不自然**：调整光照和透视相关的描述
- **角色一致性问题**：使用更具体的特征描述
- **色彩不协调**：在提示中明确指定色彩方案

## 结论

Nano Banana (Gemini 2.5 Flash Image Preview) 代表了 AI 图像生成和编辑领域的重大进步。其强大的多图像组合能力、出色的角色一致性维护，以及通过自然语言进行的迭代式改进，使其成为内容创作者、设计师和开发者的强大工具。

通过掌握本文档中介绍的 API 调用方法和 Prompt Remix 技术，用户可以充分发挥这一先进模型的潜力，创造出高质量、专业水准的视觉内容。随着模型的持续发展和优化，我们可以期待更多创新功能和应用场景的出现。
