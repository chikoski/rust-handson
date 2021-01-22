const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");
const markdownIt = require("markdown-it");
const createSlide = require("./plugins/create-slide");

module.exports = function(eleventyConfig) {
  eleventyConfig.addPlugin(syntaxHighlight);
  eleventyConfig.addPassthroughCopy("assets");

  eleventyConfig.setLibrary("md", markdownIt({}).use(createSlide));
  return {
    passthroughFileCopy: true,
  };
};
