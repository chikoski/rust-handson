const TagsToCreateHolizontalSlide = ["h1", "h2", "h3"];

class SectionFactory{
  constructor(Token){
    this.createToken = (kind, attributes = []) => {
      let nest = 1;
      if(kind === "close"){
        nest = -1;
      }
      const token = new Token("section_" + kind, "section", nest);
      token.block = true;
      token.attributes = attributes.map(attribute => [attribute[0], attribute[1]]);
      return token;
    }
  }
  open(attributes = []){
    return this.createToken("open", attributes);
  }
  close(){
    return this.createToken("close");
  }
}

class SlideCreator{
  constructor(state){
    this.state = state;
    this.factory = new SectionFactory(state.Token);
    
    this.tokens = [];
    this.verticalSlides = [];
  }
  addVerticalSlide(){
    this.verticalSlides.push([this.factory.open()]);
  }
  closeLastVerticalSlide(){
    if(this.verticalSlides.length > 0){
      const token = this.factory.close();
      this.getLastVerticalSlide().push(token);
    }
  }
  addHolizontalSlide(){
    this.addVerticalSlide();
  }
  closeHolizontalSlide(){
    this.closeLastVerticalSlide();
    if(this.verticalSlides.length === 1){
      this.tokens.push(this.getLastVerticalSlide());
      this.verticalSlides = [];
    }else if(this.verticalSlides.length > 1){
      const tokens = this.verticalSlides.flat();
      for(const token of tokens){
        token.level += 1;
      }
      tokens.unshift(this.factory.open());
      tokens.push(this.factory.close());
      this.tokens.push(tokens);
      this.verticalSlides = [];
    }
  }
  getLastVerticalSlide(){
    return this.verticalSlides[this.verticalSlides.length - 1];
  }
  build(){
    for(const token of this.state.tokens){
      if(token.type === "heading_open"){
        if(TagsToCreateHolizontalSlide.includes(token.tag)){
          this.closeHolizontalSlide();
          this.addHolizontalSlide();
        }else{
          this.closeLastVerticalSlide();
          this.addVerticalSlide();          
        }
      }
      this.getLastVerticalSlide().push(token);
    }
    this.closeHolizontalSlide();
    return this.tokens.flat();
  }
}

function createSlide(state){
  const creator = new SlideCreator(state);
  state.tokens = creator.build();
}

module.exports = function(md){
  md.core.ruler.push("create_slide", createSlide);
}