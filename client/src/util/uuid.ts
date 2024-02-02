import crypto from 'crypto';

const generateUUIDv4 = () => {
    return [1e7].concat(-1e3, -4e3, -8e3, -1e11).join('').replace(/[018]/g, c =>
        (Number(c) ^ crypto.randomBytes(1)[0] % 16 >> Number(c) / 4).toString(16)
    );
};

export default generateUUIDv4;