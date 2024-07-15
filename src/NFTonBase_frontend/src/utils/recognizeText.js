import Tesseract from 'tesseract.js';
import { resizeAndGrayscaleImage } from './resizeAndGrayscaleImage';

const preprocessText = (text) => {
    const numberPattern = /(\d+)\.(\d{3})(,\d+)?/g;
    const normalizeNumber = (match, wholeNumber, thousands, decimals) => {
      return `${wholeNumber}${thousands}${decimals ? decimals.replace(',', '.') : ''}`;
    };
    return text.replace(numberPattern, normalizeNumber);
};

export const recognizeText = async (file) => {
    const worker = await Tesseract.createWorker('eng+chi_tra+rus');
    
    try {
        const processedImage = await resizeAndGrayscaleImage(file);
        const { data: { text } } = await worker.recognize(processedImage);
        return text;
    } catch (error) {
        return 'Error of bild recognize:' + error;
    } finally {
        await worker.terminate();
    }
};
