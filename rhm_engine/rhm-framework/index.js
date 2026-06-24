const { execSync } = require('child_process');

function rhm(strings, ...values) {
    return strings.reduce((acc, str, i) => acc + str + (values[i] || ''), '');
}

function vitePluginRhm() {
    return {
        name: 'vite-plugin-rhm',
        transform(src, id) {
            if (id.endsWith('.js') || id.endsWith('.jsx') || id.endsWith('.ts') || id.endsWith('.tsx')) {
                if (src.includes('rhm`')) {
                    const updatedSrc = src.replace(/rhm`([\s\S]*?)`/g, (match, rhmContent) => {
                        try {
                            const escapedContent = rhmContent.replace(/"/g, '\\"');
                            const output = execSync(`rhm -e "${escapedContent}"`).toString();
                            const cleanHtml = output
                                .split('\n')
                                .filter(line => !line.includes('[ANALYTICS]') && !line.includes('[TIME]') && !line.includes('[PROFILE]'))
                                .join('\n')
                                .trim();
                            return JSON.stringify(cleanHtml);
                        } catch (err) {
                            return `"[RHM COMPILER ERROR INLINE]"`;
                        }
                    });
                    return {
                        code: updatedSrc,
                        map: null
                    };
                }
            }
        }
    };
}

module.exports = { rhm, vitePluginRhm };